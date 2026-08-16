//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2361/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2361(t1845: f64, t3734: f64, t24995: f64, t8643: f64, t23831: f64, t7458: f64, t22480: f64, t7461: f64, t9348: f64, t12724: f64, t12823: f64, t12835: f64, t1976: f64, t2314: f64, t24980: f64, t25965: f64, t3929: f64, t4034: f64, t6517: f64, t7472: f64, t7681: f64, t91666: f64, t91671: f64, t91673: f64, t91674: f64, t91678: f64, t91681: f64, t91684: f64, t91690: f64, t91694: f64) -> f64 {
    let t91695 = t1845 * t3734;
    let t91698 = 6.0_f64 * t24995 * t8643 * t91695;
    let t91704 = 2.0_f64 * t7458 * t23831;
    let t91706 = 2.0_f64 * t7458 * t22480;
    let t91708 = 2.0_f64 * t9348 * t7461;
    let t91709 = -t12724 * t1976 - 2.0_f64 * t12823 * t7472 - 2.0_f64 * t12835 * t6517 - 4.0_f64 * t2314 * t24980 - 4.0_f64 * t25965 * t4034 + t3929 * t7681 + t91666 + t91671 - t91673 - t91674 + t91678 + t91681 - t91684 - t91690 - t91694 - t91698 - t91704 - t91706 - t91708;
    t91709
}
