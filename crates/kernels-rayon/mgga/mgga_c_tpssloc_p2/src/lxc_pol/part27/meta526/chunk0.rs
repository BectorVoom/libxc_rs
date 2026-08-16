//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1935/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1935(t25749: f64, t884: f64, t23329: f64, t225: f64, t7594: f64, t254: f64, t382: f64, t10164: f64, t1955: f64, t4664: f64, t1052: f64, t1066: f64, t14529: f64, t1635: f64, t1956: f64, t23327: f64, t23346: f64, t23359: f64, t23372: f64, t25447: f64, t25450: f64, t25453: f64, t25732: f64, t25736: f64, t25739: f64, t25743: f64, t3026: f64, t6687: f64, t7557: f64, t7600: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25750 = t25749 * t884;
    let t25751 = t23329 * t25750;
    let t25755 = t7594 * t225;
    let t25757 = t382 * t254;
    let t25758 = t10164 * t1955;
    let t25759 = t25758 * t4664;
    let t25762 = 0.82246703342411321825e-2_f64 * t6687 * t25447 + 0.91385225936012579807e-3_f64 * t25450 + 2.0_f64 * t1052 * t25453 - t1052 * t25732 + 0.21932454224643019153e-1_f64 * t23346 * t7557 - 0.27415567780803773942e-2_f64 * t25736 + 0.16449340668482264365e-1_f64 * t6687 * t25739 + 2.0_f64 * t1052 * t25743 - t23359 - t23372 * t1635 + 2.0_f64 * t3026 * t7600 - 0.27415567780803773942e-2_f64 * t23327 * t25751 - t14529 * t1956 - t25755 * t1066 - 6.0_f64 * t25757 * t25759;
    (t25750, t25751, t25755, t25757, t25758, t25759, t25762)
}
