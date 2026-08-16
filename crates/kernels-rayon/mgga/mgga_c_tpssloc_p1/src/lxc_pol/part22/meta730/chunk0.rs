//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2396/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2396(t21185: f64, t41935: f64, t896: f64, t17210: f64, t4370: f64, t13629: f64, t5705: f64, t17271: f64, t4362: f64, t41942: f64, t17218: f64, t41962: f64, t47787: f64, t59700: f64, t59702: f64, t59704: f64, t60274: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t68619 = t41935 * t21185 * t896;
    let t68626 = t17210 * t4370;
    let t68628 = t13629 * t5705;
    let t68630 = t4362 * t17271;
    let t68633 = t41942 * t21185 * t896;
    let t68635 = t17218 * t4370;
    let t68637 = t41962 - 0.485484375e1_f64 * t68619 + 0.5519e-1_f64 * t60274 - 0.12077e1_f64 * t59700 + 0.40256666666666666666e0_f64 * t59702 + 0.33547222222222222222e0_f64 * t59704 + 0.93932222222222222225e0_f64 * t47787 + 0.58258125e1_f64 * t68626 - 0.3883875e1_f64 * t68628 - 0.3883875e1_f64 * t68630 + 0.6189328125e-1_f64 * t68633 - 0.1237865625e0_f64 * t68635;
    (t68619, t68626, t68628, t68630, t68633, t68635, t68637)
}
