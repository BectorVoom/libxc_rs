//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1170/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1170<F: Float>(t1616: F, t7492: F, t29280: F, t4142: F, t102294: F, t7978: F, t102029: F, t102088: F, t102137: F, t102371: F, t1307: F, t18187: F, t21655: F, t27567: F, t27583: F, t28721: F, t28738: F, t28765: F, t28807: F, t4440: F, t7968: F, t99331: F, t99424: F, t99437: F) -> (F, F) {
    let t102412 = t1616 * t7492;
    let t102421 = t4142 * t29280;
    let t102425 = t7978 * t102294;
    let t102427 = 0.11584201388888888889e-3 * t27583 * t102088 - 0.61782407407407407407e-3 * t99331 * t28807 - 0.61836467013888888888e-4 * t27567 * t102137 + t99424 + 0.61782407407407407408e-3 * t27583 * t18187 * t28765 * t21655 + t99437 + 0.11584201388888888889e-3 * t27583 * t4440 * t102412 * t1307 - 0.92754700520833333334e-4 * t7968 * t102029 - 0.92754700520833333334e-4 * t7968 * t102371 + 0.10317654320987654321e-2 * t102421 - 0.92754700520833333334e-4 * t28721 * t28738 - 0.23168402777777777778e-3 * t102425;
    (t102421, t102427)
}
