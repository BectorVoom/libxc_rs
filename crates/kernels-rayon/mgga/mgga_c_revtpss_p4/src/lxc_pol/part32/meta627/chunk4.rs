//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2006/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2006(t198: f64, t8034: f64, t2411: f64, t30419: f64, t105898: f64, t105919: f64, t105924: f64, t106555: f64, t106566: f64, t106569: f64, t106611: f64, t106618: f64, t106626: f64, t1940: f64, t2071: f64, t2403: f64, t26425: f64, t26585: f64, t27173: f64, t27385: f64, t28291: f64, t28472: f64, t29716: f64, t30317: f64, t50080: f64, t5824: f64, t7092: f64, t7428: f64, t8020: f64) -> (f64, f64, f64) {
    let t110165 = t198 * t8034;
    let t110177 = t30419 * t2411;
    let t110196 = 3.0_f64 * t50080 * t30317 - 3.0_f64 / 2.0_f64 * t26425 * t105924 - 3.0_f64 * t28472 * t106566 + 2.0_f64 * t110165 * t27385 + t1940 * t7428 * t5824 / 2.0_f64 + 3.0_f64 * t2403 * t8020 * t27173 + 3.0_f64 / 2.0_f64 * t2403 * t2071 * t105898 - t1940 * t110177 * t7092 / 2.0_f64 + 2.0_f64 * t28472 * t106555 + 3.0_f64 / 2.0_f64 * t2403 * t2071 * t105919 + 3.0_f64 / 2.0_f64 * t2403 * t2071 * t106618 - 3.0_f64 * t26425 * t106626 - t1940 * t26585 * t29716 - 3.0_f64 * t28291 * t106569 + t28472 * t106611;
    (t110165, t110177, t110196)
}
