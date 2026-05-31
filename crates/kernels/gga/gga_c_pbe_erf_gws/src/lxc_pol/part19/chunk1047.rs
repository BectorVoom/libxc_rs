//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1047/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1047<F: Float>(t11757: F, t905: F, t8996: F, t9016: F, t11744: F, t858: F, t3065: F, t8978: F, t3134: F, t8881: F, t8983: F, t8897: F) -> (F, F, F, F, F, F, F) {
    let t11758 = t905 * t11757;
    let t11762 = t9016 * t8996 / F::cast_from(48.0_f64);
    let t11763 = t858 * t11744;
    let t11764 = t3065 * t11763;
    let t11766 = t8978 * t11764 / F::cast_from(96.0_f64);
    let t11768 = t8881 * t3134 / F::cast_from(48.0_f64);
    let t11770 = t8978 * t8983 / F::cast_from(48.0_f64);
    let t11772 = t9016 * t8897 / F::cast_from(24.0_f64);
    (t11758, t11762, t11764, t11766, t11768, t11770, t11772)
}
