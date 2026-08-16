//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1418/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1418<F: Float>(t1315: F, t1453: F, t1847: F, t1911: F, t21814: F, t21882: F, t21891: F, t22506: F, t22525: F, t2322: F, t4248: F, t4254: F, t4293: F, t4297: F, t508: F, t511: F, t5528: F, t569: F, t5787: F, t5887: F, t649: F, t651: F, t6765: F, t6773: F, t6934: F, t7732: F) -> F {
    let t22531 = t1315 * t6934 + t1453 * t6773 + F::cast_from(2.0_f64) * t1847 * t5787 + F::cast_from(2.0_f64) * t1911 * t5528 - t21814 * t508 - F::cast_from(2.0_f64) * t21882 * t651 - F::cast_from(4.0_f64) * t21891 * t651 + t22506 * t511 + t22525 * t569 - F::cast_from(4.0_f64) * t2322 * t5887 - F::cast_from(4.0_f64) * t4248 * t4293 - F::cast_from(4.0_f64) * t4248 * t4297 - F::cast_from(4.0_f64) * t4254 * t5887 - F::cast_from(4.0_f64) * t4293 * t7732 - t649 * t6765;
    t22531
}
