//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2036/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2036<F: Float>(t102854: F, t103586: F, t105923: F, t106561: F, t106625: F, t110177: F, t1544: F, t1583: F, t18392: F, t18875: F, t1940: F, t2071: F, t2403: F, t26585: F, t26590: F, t27384: F, t28456: F, t28460: F, t29598: F, t30439: F, t4343: F, t4433: F, t4537: F, t4541: F, t50080: F, t5966: F, t6079: F, t7428: F, t7432: F, t8020: F, t890: F, t95976: F) -> F {
    let t110839 = F::cast_from(6.0_f64) * t2403 * t26590 * t106561 + F::cast_from(3.0_f64) * t2403 * t2071 * t18392 + F::cast_from(4.0_f64) * t1940 * t103586 * t27384 - F::cast_from(6.0_f64) * t2403 * t26585 * t29598 - F::cast_from(6.0_f64) * t2403 * t28460 * t18875 + F::cast_from(6.0_f64) * t2403 * t28456 * t1544 - F::cast_from(6.0_f64) * t2403 * t7432 * t106625 - F::cast_from(2.0_f64) * t1940 * t28460 * t4537 + F::cast_from(12.0_f64) * t4541 * t8020 * t4433 - F::cast_from(3.0_f64) * t2403 * t7432 * t105923 - F::cast_from(2.0_f64) * t1940 * t102854 * t1583 - t1940 * t110177 * t890 + F::cast_from(6.0_f64) * t4541 * t7428 * t5966 + F::cast_from(2.0_f64) * t1940 * t95976 * t6079 + F::cast_from(6.0_f64) * t2403 * t8020 * t4343 + F::cast_from(6.0_f64) * t50080 * t30439;
    t110839
}
