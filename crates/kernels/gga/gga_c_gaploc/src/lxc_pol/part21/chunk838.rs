//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 838/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk838<F: Float>(t475: F, t8097: F, t1445: F, t7980: F, t1265: F, t2778: F, t7996: F, t1450: F, t1456: F, t1549: F, t1562: F, t1580: F, t1625: F, t1641: F, t1646: F, t2843: F, t2847: F, t2851: F, t2856: F, t2872: F, t2877: F, t4679: F, t4762: F, t4953: F, t536: F, t574: F, t597: F, t8063: F, t8072: F, t8077: F, t8080: F, t8084: F, t8087: F, t8090: F) -> F {
    let t8098 = t8097 * t475;
    let t8099 = t1445 * t8098;
    let t8104 = t7980 * t475;
    let t8105 = t1445 * t8104;
    let t8108 = t2778 * t1265;
    let t8109 = t1445 * t8108;
    let t8114 = t1445 * t7996;
    let t8117 = F::cast_from(0.47667319935800568892e0_f64) * t536 * t8063 + F::cast_from(0.71500979903700853338e0_f64) * t1549 * t2877 + F::cast_from(0.35750489951850426669e0_f64) * t1625 * t2877 + F::cast_from(0.71500979903700853338e0_f64) * t536 * t8072 + F::cast_from(0.71500979903700853338e0_f64) * t4679 * t2843 + F::cast_from(0.71500979903700853338e0_f64) * t1456 * t8077 + F::cast_from(0.35750489951850426669e0_f64) * t1456 * t8080 - F::cast_from(0.23005755572352449806e1_f64) * t1450 * t8084 - F::cast_from(0.71500979903700853338e0_f64) * t8087 * t1646 - F::cast_from(0.71500979903700853338e0_f64) * t8090 * t1646 - F::cast_from(0.35750489951850426669e0_f64) * t2872 * t4762 - F::cast_from(0.13803453343411469884e2_f64) * t4953 * t2856 - F::cast_from(0.13803453343411469884e2_f64) * t1562 * t8099 - F::cast_from(0.92023022289409799224e1_f64) * t1641 * t2847 - F::cast_from(0.92023022289409799224e1_f64) * t574 * t8105 - F::cast_from(0.46011511144704899612e1_f64) * t574 * t8109 + F::cast_from(0.23005755572352449806e2_f64) * t1580 * t2851 + F::cast_from(0.23005755572352449806e2_f64) * t597 * t8114;
    t8117
}
