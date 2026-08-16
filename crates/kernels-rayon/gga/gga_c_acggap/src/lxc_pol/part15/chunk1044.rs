//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1044/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1044(t33682: f64, t8337: f64, t2404: f64, t7924: f64, t33839: f64, t33841: f64, t33843: f64, t33851: f64, t33859: f64, t33886: f64, t33903: f64, t33940: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36809 = t33682 * t8337;
    let t36811 = t7924 * t2404;
    let t36817 = 0.31448092289604152068e-2_f64 * t33839;
    let t36818 = 0.37737710747524982482e-2_f64 * t33841;
    let t36819 = 0.62896184579208304138e-3_f64 * t33843;
    let t36821 = 0.41930789719472202758e-3_f64 * t33851;
    let t36824 = 11.0_f64 / 288.0_f64 * t33859;
    let t36836 = 0.57165357490759649296e-3_f64 * t33886;
    let t36841 = 0.57165357490759649296e-3_f64 * t33903;
    let t36870 = 0.21437009059034868486e-2_f64 * t33940;
    (t36809, t36811, t36817, t36818, t36819, t36821, t36824, t36836, t36841, t36870)
}
