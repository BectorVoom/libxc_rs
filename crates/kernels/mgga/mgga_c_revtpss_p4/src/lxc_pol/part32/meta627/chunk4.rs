//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2006/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2006<F: Float>(t198: F, t8034: F, t2411: F, t30419: F, t105898: F, t105919: F, t105924: F, t106555: F, t106566: F, t106569: F, t106611: F, t106618: F, t106626: F, t1940: F, t2071: F, t2403: F, t26425: F, t26585: F, t27173: F, t27385: F, t28291: F, t28472: F, t29716: F, t30317: F, t50080: F, t5824: F, t7092: F, t7428: F, t8020: F) -> (F, F, F) {
    let t110165 = t198 * t8034;
    let t110177 = t30419 * t2411;
    let t110196 = F::new(3.0) * t50080 * t30317 - F::new(3.0) / F::new(2.0) * t26425 * t105924 - F::new(3.0) * t28472 * t106566 + F::new(2.0) * t110165 * t27385 + t1940 * t7428 * t5824 / F::new(2.0) + F::new(3.0) * t2403 * t8020 * t27173 + F::new(3.0) / F::new(2.0) * t2403 * t2071 * t105898 - t1940 * t110177 * t7092 / F::new(2.0) + F::new(2.0) * t28472 * t106555 + F::new(3.0) / F::new(2.0) * t2403 * t2071 * t105919 + F::new(3.0) / F::new(2.0) * t2403 * t2071 * t106618 - F::new(3.0) * t26425 * t106626 - t1940 * t26585 * t29716 - F::new(3.0) * t28291 * t106569 + t28472 * t106611;
    (t110165, t110177, t110196)
}
