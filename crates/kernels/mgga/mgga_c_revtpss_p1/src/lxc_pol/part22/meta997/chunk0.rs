//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3387/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3387<F: Float>(t15393: F, t15421: F, t15397: F, t52224: F, t2918: F, t2924: F, t6110: F, t11385: F, t2875: F, t6145: F, t198: F, t3336: F, t336: F, t63589: F, t63592: F, t63596: F, t63600: F, t63601: F, t63607: F, t63609: F, t63612: F, t63615: F, t63618: F) -> (F, F, F, F, F) {
    let t63620 = F::cast_from(0.32163958997385070134e2_f64) * t15421 * t15393;
    let t63622 = F::cast_from(0.1034520258385468006e4_f64) * t52224 * t15397;
    let t63625 = F::new(6.0) * t2924 * t6110 * t2918;
    let t63628 = F::cast_from(0.57895126195293126241e3_f64) * t11385 * t6145 * t2875;
    let t63629 = -F::new(2.0) * t198 * t3336 * t336 * t63601 + t63589 + t63592 + t63596 + t63600 - t63607 + t63609 + t63612 + t63615 - t63618 + t63620 + t63622 + t63625 + t63628;
    (t63620, t63622, t63625, t63628, t63629)
}
