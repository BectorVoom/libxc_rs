//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 586/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk586<F: Float>(t1564: F, t3689: F, t475: F, t1445: F, t10331: F, t10334: F, t10336: F, t10350: F, t10353: F, t10356: F, t10358: F, t12045: F, t12048: F, t12051: F, t12054: F, t12057: F, t12060: F, t12065: F, t1562: F, t1572: F, t1646: F, t2386: F, t536: F, t567: F, t574: F, t597: F) -> (F, F) {
    let t12068 = t1564 * t3689;
    let t12069 = t12068 * t475;
    let t12070 = t1445 * t12069;
    let t12073 = t10331 + t10334 + t10336 - t10350 - 0.46011511144704899612e1 * t574 * t12045 + 0.11502877786176224903e2 * t597 * t12048 + 0.23005755572352449806e1 * t567 * t12051 - 0.10725146985555128001e1 * t12054 * t2386 + 0.71500979903700853338e0 * t1572 * t12057 + t10353 - t10356 - 0.35750489951850426669e0 * t12060 * t1646 + 0.35750489951850426669e0 * t536 * t12065 - 0.69017266717057349418e1 * t1562 * t12070 - t10358;
    (t12068, t12073)
}
