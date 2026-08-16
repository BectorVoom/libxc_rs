//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 838/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk838(t475: f64, t8097: f64, t1445: f64, t7980: f64, t1265: f64, t2778: f64, t7996: f64, t1450: f64, t1456: f64, t1549: f64, t1562: f64, t1580: f64, t1625: f64, t1641: f64, t1646: f64, t2843: f64, t2847: f64, t2851: f64, t2856: f64, t2872: f64, t2877: f64, t4679: f64, t4762: f64, t4953: f64, t536: f64, t574: f64, t597: f64, t8063: f64, t8072: f64, t8077: f64, t8080: f64, t8084: f64, t8087: f64, t8090: f64) -> f64 {
    let t8098 = t8097 * t475;
    let t8099 = t1445 * t8098;
    let t8104 = t7980 * t475;
    let t8105 = t1445 * t8104;
    let t8108 = t2778 * t1265;
    let t8109 = t1445 * t8108;
    let t8114 = t1445 * t7996;
    let t8117 = 0.47667319935800568892e0_f64 * t536 * t8063 + 0.71500979903700853338e0_f64 * t1549 * t2877 + 0.35750489951850426669e0_f64 * t1625 * t2877 + 0.71500979903700853338e0_f64 * t536 * t8072 + 0.71500979903700853338e0_f64 * t4679 * t2843 + 0.71500979903700853338e0_f64 * t1456 * t8077 + 0.35750489951850426669e0_f64 * t1456 * t8080 - 0.23005755572352449806e1_f64 * t1450 * t8084 - 0.71500979903700853338e0_f64 * t8087 * t1646 - 0.71500979903700853338e0_f64 * t8090 * t1646 - 0.35750489951850426669e0_f64 * t2872 * t4762 - 0.13803453343411469884e2_f64 * t4953 * t2856 - 0.13803453343411469884e2_f64 * t1562 * t8099 - 0.92023022289409799224e1_f64 * t1641 * t2847 - 0.92023022289409799224e1_f64 * t574 * t8105 - 0.46011511144704899612e1_f64 * t574 * t8109 + 0.23005755572352449806e2_f64 * t1580 * t2851 + 0.23005755572352449806e2_f64 * t597 * t8114;
    t8117
}
