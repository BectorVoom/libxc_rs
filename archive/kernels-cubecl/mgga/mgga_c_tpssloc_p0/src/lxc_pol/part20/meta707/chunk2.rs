//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2700/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2700<F: Float>(t16448: F, t225: F, t12020: F, t1842: F, t12023: F, t12026: F, t12030: F, t1372: F, t1375: F, t1385: F, t1386: F, t16022: F, t16030: F, t16122: F, t16436: F, t16439: F, t16475: F, t26224: F, t3882: F, t3887: F, t3889: F, t3911: F, t3912: F, t5215: F, t5326: F, t5353: F, t5354: F, t568: F) -> F {
    let t55093 = t16448 * t225;
    let t55118 = t12020 * t1842;
    let t55124 = F::cast_from(6.0_f64) * t1375 * t1385 * t16436 * t3887 + F::cast_from(6.0_f64) * t1375 * t3887 * t3911 * t5353 - F::cast_from(18.0_f64) * t12026 * t26224 * t55118 + F::cast_from(3.0_f64) * t1372 * t16122 * t568 - F::cast_from(6.0_f64) * t12023 * t5215 + F::cast_from(6.0_f64) * t12030 * t5326 - F::cast_from(3.0_f64) * t12030 * t5354 - F::cast_from(6.0_f64) * t1386 * t55093 - F::cast_from(3.0_f64) * t16022 * t3912 - F::cast_from(3.0_f64) * t16030 * t3912 + F::cast_from(6.0_f64) * t16439 * t3889 - F::cast_from(3.0_f64) * t16439 * t3912 - F::cast_from(18.0_f64) * t16475 * t3882;
    t55124
}
