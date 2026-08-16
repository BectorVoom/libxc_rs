//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3099/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3099<F: Float>(t12772: F, t17736: F, t17738: F, t1042: F, t1261: F, t12646: F, t12855: F, t12926: F, t12956: F, t13095: F, t17204: F, t17214: F, t17261: F, t17344: F, t17589: F, t17710: F, t17732: F, t1789: F, t21275: F, t3647: F, t3720: F, t44200: F, t44215: F, t44500: F, t45796: F, t5268: F, t5332: F, t53459: F, t53464: F, t5391: F, t56825: F, t56830: F, t56835: F, t56838: F, t56853: F, t56861: F) -> F {
    let t56867 = t17736 * t12772 * t17738;
    let t56873 = -F::cast_from(0.57165357490759649295e-3_f64) * t44200 - F::cast_from(0.12862205435420921092e-2_f64) * t12855 * t3720 * t5332 * t45796 - F::cast_from(0.12862205435420921092e-2_f64) * t12855 * t3720 * t5332 * t56825 - F::cast_from(0.38586616306262763275e-2_f64) * t44500 * t3720 * t17710 * t56830 + F::cast_from(0.30488190661738479624e-2_f64) * t56835 + F::cast_from(0.47637797908966374414e-3_f64) * t56838 - F::cast_from(0.25724410870841842183e-2_f64) * t3647 * t17204 - F::cast_from(0.85748036236139473944e-3_f64) * t1261 * t1042 * t5268 * t53459 + F::cast_from(0.12862205435420921092e-2_f64) * t21275 * t13095 - F::cast_from(0.85748036236139473944e-3_f64) * t17261 * t17214 + F::cast_from(0.85748036236139473944e-3_f64) * t12956 * t17589 + F::cast_from(0.57165357490759649295e-3_f64) * t56853 - F::cast_from(0.85748036236139473944e-3_f64) * t1261 * t1042 * t5268 * t53464 - F::cast_from(0.28582678745379824648e-3_f64) * t44215 + F::cast_from(0.17149607247227894789e-2_f64) * t56861 * t17732 + F::cast_from(0.45732285992607719436e-2_f64) * t5391 * t12926 - F::cast_from(0.11433071498151929859e-2_f64) * t56867 - F::cast_from(0.38586616306262763276e-2_f64) * t17344 * t1042 * t1789 * t12646;
    t56873
}
