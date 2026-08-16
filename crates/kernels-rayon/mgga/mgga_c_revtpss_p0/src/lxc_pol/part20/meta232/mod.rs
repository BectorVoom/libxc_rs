//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta232 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1028;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1029;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta232(t2482: f64, t27: f64, t2719: f64, t221: f64, t2485: f64, t2724: f64, t2741: f64, t2756: f64, t820: f64, t843: f64, t2726: f64, t10665: f64, t2723: f64, t827: f64, t828: f64, t821: f64, t235: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10850, t10852, t10853, t10855, t10858, t10859, t10861) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1028(t2482, t27, t2719, t221, t2485, t2724, t2741, t2756, t820, t843, t2726, t10665, t2723);
        let (t10863, t10866, t10867, t10868) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1029(t10861, t827, t828, t821, t235);
    (t10850, t10852, t10853, t10855, t10858, t10859, t10861, t10863, t10866, t10867, t10868)
}
