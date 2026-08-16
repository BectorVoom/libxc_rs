//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1003/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1003(t106: f64, t12063: f64, t192: f64, t1564: f64, t3689: f64, t475: f64, t1445: f64, t10331: f64, t10334: f64, t10336: f64, t10350: f64, t10353: f64, t10356: f64, t10358: f64, t12045: f64, t12048: f64, t12051: f64, t12054: f64, t12057: f64, t12060: f64, t1562: f64, t1572: f64, t1646: f64, t2386: f64, t536: f64, t567: f64, t574: f64, t597: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12064 = t12063 * t106;
    let t12065 = t12064 * t192;
    let t12068 = t1564 * t3689;
    let t12069 = t12068 * t475;
    let t12070 = t1445 * t12069;
    let t12073 = t10331 + t10334 + t10336 - t10350 - 0.46011511144704899612e1_f64 * t574 * t12045 + 0.11502877786176224903e2_f64 * t597 * t12048 + 0.23005755572352449806e1_f64 * t567 * t12051 - 0.10725146985555128001e1_f64 * t12054 * t2386 + 0.71500979903700853338e0_f64 * t1572 * t12057 + t10353 - t10356 - 0.35750489951850426669e0_f64 * t12060 * t1646 + 0.35750489951850426669e0_f64 * t536 * t12065 - 0.69017266717057349418e1_f64 * t1562 * t12070 - t10358;
    (t12064, t12065, t12068, t12069, t12070, t12073)
}
