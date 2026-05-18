//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1129/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1129<F: Float>(t13920: F, t543: F, t1390: F, t828: F, t1398: F, t1882: F, t3938: F, t13789: F, t13869: F, t13874: F, t1388: F, t13880: F, t1410: F, t3934: F, t9753: F, t9762: F, t9766: F, t9771: F, t9776: F, t9780: F, t9786: F, t9791: F) -> (F, F, F) {
    let t13921 = t13920 * t543;
    let t13923 = t1390 * t828 * t13921;
    let t13926 = t1882 * t1398;
    let t13927 = t13926 * t3938;
    let t13928 = t13789 * t13927;
    let t13931 = -F::new(0.20007875121765877254e-1) * t9753 - F::new(0.50820002809285328224e-4) * t9762 + F::new(0.10841600599314203354e-2) * t9766 + F::new(0.71456696863449561619e-5) * t9771 - F::new(0.15244095330869239812e-3) * t9776 - F::new(0.45351183609335988442e-1) * t9780 + F::new(0.85748036236139473944e-2) * t1410 * t13869 + F::new(0.42874018118069736972e-2) * t1410 * t13874 + t13880 - F::new(0.21437009059034868486e-3) * t1388 * t13923 + F::new(0.17149607247227894789e-2) * t3934 * t13928 - t9786 - t9791;
    (t13921, t13926, t13931)
}
