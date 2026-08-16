//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1432/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1432<F: Float>(t104740: F, t104749: F, t104818: F, t106758: F, t106800: F, t106813: F, t106816: F, t106842: F, t106849: F, t1409: F, t1860: F, t1864: F, t20217: F, t20234: F, t20245: F, t2109: F, t2110: F, t21510: F, t24498: F, t24514: F, t26016: F, t27298: F, t27356: F, t27956: F, t29474: F, t5392: F, t5398: F, t56: F, t67: F, t7246: F, t7251: F, t7445: F, t7974: F, t83803: F, t85539: F, t90137: F, t96157: F, t96443: F) -> F {
    let t108983 = -F::cast_from(5.0_f64) * t26016 * t104740 - F::cast_from(15.0_f64) * t24514 * t106758 + F::cast_from(30.0_f64) * t90137 * t104749 - F::cast_from(10.0_f64) * t96443 * t27298 - t1860 * (-F::cast_from(1232.0_f64) / F::cast_from(27.0_f64) * t20245 * t56 - F::cast_from(220.0_f64) / F::cast_from(9.0_f64) * t104818 * t1409 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t96157 * t5392 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t27356 * t5398 + F::cast_from(5.0_f64) / F::cast_from(108.0_f64) * t85539 * t20234 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t24498 * t21510 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t7251 * t20217 + t83803) * t67 * t1864 / F::cast_from(6.0_f64) - t1860 * t29474 * t7445 / F::cast_from(2.0_f64) - t1860 * t7974 * t27956 / F::cast_from(2.0_f64) - t1860 * t2109 * t106800 / F::cast_from(6.0_f64) + t106816 * t2110 + F::cast_from(5.0_f64) / F::cast_from(2.0_f64) * t7246 * t106813 + F::cast_from(5.0_f64) / F::cast_from(2.0_f64) * t7246 * t106842 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t7246 * t106849;
    t108983
}
