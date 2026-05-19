//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1001/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1001<F: Float>(t106: F, t12063: F, t192: F, t1564: F, t3689: F, t475: F, t1445: F, t10331: F, t10334: F, t10336: F, t10350: F, t10353: F, t10356: F, t10358: F, t12045: F, t12048: F, t12051: F, t12054: F, t12057: F, t12060: F, t1562: F, t1572: F, t1646: F, t2386: F, t536: F, t567: F, t574: F, t597: F) -> (F, F, F, F, F, F) {
    let t12064 = t12063 * t106;
    let t12065 = t12064 * t192;
    let t12068 = t1564 * t3689;
    let t12069 = t12068 * t475;
    let t12070 = t1445 * t12069;
    let t12073 = t10331 + t10334 + t10336 - t10350 - F::cast_from(0.46011511144704899612e1_f64) * t574 * t12045 + F::cast_from(0.11502877786176224903e2_f64) * t597 * t12048 + F::cast_from(0.23005755572352449806e1_f64) * t567 * t12051 - F::cast_from(0.10725146985555128001e1_f64) * t12054 * t2386 + F::cast_from(0.71500979903700853338e0_f64) * t1572 * t12057 + t10353 - t10356 - F::cast_from(0.35750489951850426669e0_f64) * t12060 * t1646 + F::cast_from(0.35750489951850426669e0_f64) * t536 * t12065 - F::cast_from(0.69017266717057349418e1_f64) * t1562 * t12070 - t10358;
    (t12064, t12065, t12068, t12069, t12070, t12073)
}
