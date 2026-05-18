//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 907/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk907<F: Float>(t1621: F, t7920: F, t1620: F, t256: F, t7867: F, t7870: F, t7873: F, t7876: F, t7880: F, t7884: F, t7887: F, t7890: F, t7894: F, t7898: F, t7903: F, t7905: F, t7910: F, t7915: F, t7917: F, t7919: F) -> (F, F) {
    let t7921 = t1621 * t7920;
    let t7923 = F::new(4.0) / F::new(15.0) * t1620 * t7921;
    let t7924 = -t7867 + t7870 - t7873 - t7876 + t7880 - t7884 - t7887 + t7890 + t7894 + t7898 + t7903 - t7905 + t7910 * t256 / F::new(3.0) - t7915 - t7917 + t7919 - t7923;
    (t7923, t7924)
}
