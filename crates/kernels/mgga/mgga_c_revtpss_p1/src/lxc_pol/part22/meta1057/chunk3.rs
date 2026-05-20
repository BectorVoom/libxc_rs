//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3750/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3750<F: Float>(t17693: F, t20937: F, t56756: F, t1222: F, t17240: F, t20310: F, t12832: F, t12866: F, t17170: F, t17351: F, t17353: F, t17420: F, t17513: F, t17703: F, t17705: F, t20800: F, t21049: F, t21259: F, t3603: F, t3604: F, t3611: F, t3720: F, t44510: F, t44517: F, t5332: F, t5340: F, t5401: F, t59040: F, t59043: F, t59062: F, t69839: F, t70633: F) -> F {
    let t71341 = t17693 * t56756 * t20937;
    let t71373 = t1222 * t17240 * t20310;
    let t71375 = F::cast_from(0.57165357490759649296e-3_f64) * t12866 * t59062 * t5401 - F::cast_from(0.76220476654346199061e-3_f64) * t71341 - F::cast_from(0.28582678745379824648e-3_f64) * t44517 * t69839 * t3611 * t17513 + F::cast_from(0.57165357490759649296e-3_f64) * t44510 * t69839 * t3604 * t17513 + F::cast_from(0.57165357490759649296e-3_f64) * t17351 * t17353 * t3611 * t70633 + F::cast_from(0.17149607247227894789e-2_f64) * t21049 * t17420 + F::cast_from(0.42874018118069736972e-3_f64) * t5340 * t3720 * t20800 * t17703 + F::cast_from(0.85748036236139473944e-3_f64) * t21049 * t17705 - F::cast_from(0.10162730220579493208e-2_f64) * t59040 + F::cast_from(0.85748036236139473944e-3_f64) * t5340 * t3720 * t5332 * t3603 * t17170 - F::cast_from(0.85748036236139473944e-3_f64) * t12832 * t21259 - F::cast_from(0.57165357490759649296e-3_f64) * t59043 - t71373 / F::new(108.0);
    t71375
}
