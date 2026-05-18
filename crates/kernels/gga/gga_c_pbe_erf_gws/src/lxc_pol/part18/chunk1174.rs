//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1174/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1174<F: Float>(t1167: F, t14149: F, t944: F, t3324: F, t4063: F, t14605: F, t14611: F, t14655: F, t14689: F, t14708: F, t14716: F, t14745: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14829 = t14149 * t1167;
    let t14831 = t1167 * t944;
    let t14835 = t4063 * t3324;
    let t14898 = F::new(7.0) / F::new(2304.0) * t14605;
    let t14931 = F::new(7.0) / F::new(2304.0) * t14611;
    let t14962 = F::new(7.0) / F::new(576.0) * t14655;
    let t14974 = F::new(7.0) / F::new(144.0) * t14689;
    let t14978 = F::new(7.0) / F::new(144.0) * t14708;
    let t14986 = F::new(7.0) / F::new(1152.0) * t14716;
    let t14996 = F::new(7.0) / F::new(72.0) * t14745;
    (t14829, t14831, t14835, t14898, t14931, t14962, t14974, t14978, t14986, t14996)
}
