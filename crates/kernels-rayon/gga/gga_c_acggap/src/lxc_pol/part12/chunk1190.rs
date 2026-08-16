//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1190/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1190(t35204: f64, t35210: f64, t35212: f64, t35227: f64, t35230: f64, t35238: f64, t35240: f64, t35244: f64, t35248: f64, t35250: f64, t31026: f64, t35208: f64, t35215: f64, t35219: f64, t35223: f64, t35232: f64, t35234: f64, t35242: f64) -> f64 {
    let t37435 = 0.13976929906490734252e-1_f64 * t35204;
    let t37437 = 0.18868855373762491241e-1_f64 * t35210;
    let t37438 = 0.42874018118069736972e-2_f64 * t35212;
    let t37442 = 0.28582678745379824648e-2_f64 * t35227;
    let t37443 = 0.34299214494455789578e-2_f64 * t35230;
    let t37446 = 0.21437009059034868486e-2_f64 * t35238;
    let t37447 = 0.12862205435420921092e-1_f64 * t35240;
    let t37449 = 0.85748036236139473944e-3_f64 * t35244;
    let t37450 = 0.85748036236139473944e-2_f64 * t35248;
    let t37451 = 0.32012600194825403606e-1_f64 * t35250;
    let t37453 = -t37435 + 0.31448092289604152068e-2_f64 * t35208 - t37437 + t37438 + 0.42874018118069736972e-2_f64 * t35215 + 0.42874018118069736972e-2_f64 * t35219 + 0.21437009059034868486e-2_f64 * t35223 + t37442 + t37443 + 0.34299214494455789578e-2_f64 * t35232 - 0.85748036236139473944e-3_f64 * t35234 - t37446 - t37447 - 0.51448821741683684367e-2_f64 * t35242 + t37449 + t37450 - t37451 + 0.305625e-1_f64 * t31026;
    t37453
}
