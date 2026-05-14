//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1010/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1010<F: Float>(t1843: F, t2371: F, t1310: F, t4292: F, t1518: F, t3813: F, t5517: F, t670: F, t13514: F, t508: F, t10416: F, t13435: F, t13517: F, t1453: F, t1502: F, t1519: F, t2322: F, t2328: F, t2372: F, t4248: F, t4254: F, t4257: F, t4293: F, t4297: F, t5528: F, t569: F, t651: F) -> (F,) {
    let t13521 = t1843 * t2371;
    let t13532 = t1310 * t4292;
    let t13537 = t3813 * t1518;
    let t13540 = t5517 * t670;
    let t13544 = t508 * t13514;
    let t13547 = -2.0 * t10416 * t1519 - 4.0 * t13435 * t1519 + t13517 * t569 - 2.0 * t13521 * t651 - 4.0 * t13532 * t651 - 2.0 * t13537 * t651 - 4.0 * t13540 * t651 - 2.0 * t13544 * t651 + 2.0 * t1453 * t5528 - t1502 * t3813 - 2.0 * t1843 * t2328 - 4.0 * t2322 * t4257 - 4.0 * t2322 * t4297 - 2.0 * t2372 * t4248 - 4.0 * t4254 * t4293;
    (t13547,)
}
