//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2361/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2361<F: Float>(t1845: F, t3734: F, t24995: F, t8643: F, t23831: F, t7458: F, t22480: F, t7461: F, t9348: F, t12724: F, t12823: F, t12835: F, t1976: F, t2314: F, t24980: F, t25965: F, t3929: F, t4034: F, t6517: F, t7472: F, t7681: F, t91666: F, t91671: F, t91673: F, t91674: F, t91678: F, t91681: F, t91684: F, t91690: F, t91694: F) -> F {
    let t91695 = t1845 * t3734;
    let t91698 = F::cast_from(6.0_f64) * t24995 * t8643 * t91695;
    let t91704 = F::cast_from(2.0_f64) * t7458 * t23831;
    let t91706 = F::cast_from(2.0_f64) * t7458 * t22480;
    let t91708 = F::cast_from(2.0_f64) * t9348 * t7461;
    let t91709 = -t12724 * t1976 - F::cast_from(2.0_f64) * t12823 * t7472 - F::cast_from(2.0_f64) * t12835 * t6517 - F::cast_from(4.0_f64) * t2314 * t24980 - F::cast_from(4.0_f64) * t25965 * t4034 + t3929 * t7681 + t91666 + t91671 - t91673 - t91674 + t91678 + t91681 - t91684 - t91690 - t91694 - t91698 - t91704 - t91706 - t91708;
    t91709
}
