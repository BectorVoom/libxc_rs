//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1202/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1202<F: Float>(t19095: F, t3515: F, t1230: F, t18241: F, t248: F, t11546: F, t18206: F, t11738: F, t1174: F, t1218: F, t1227: F, t1232: F, t15591: F, t15594: F, t15754: F, t1737: F, t1748: F, t19077: F, t19080: F, t19083: F, t19087: F, t19090: F, t3490: F, t4889: F, t5002: F, t5005: F, t5014: F, t5030: F, t5033: F, t6207: F, t6211: F) -> F {
    let t19096 = t3515 * t19095;
    let t19101 = t248 * t1230 * t18241;
    let t19106 = t11546 * t18206;
    let t19117 = t11738 * t19077 / F::cast_from(3072.0_f64) - t19080 * t1218 / F::cast_from(288.0_f64) + t19083 * t1232 / F::cast_from(432.0_f64) + t15754 / F::cast_from(648.0_f64) - t1174 * t19087 / F::cast_from(72.0_f64) + F::cast_from(11.0_f64) / F::cast_from(324.0_f64) * t19090 - F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t4889 * t5033 - t19096 / F::cast_from(4608.0_f64) - t3490 * t6207 / F::cast_from(4608.0_f64) - t1227 * t19101 / F::cast_from(4608.0_f64) - t3490 * t6211 / F::cast_from(2304.0_f64) - F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t1174 * t19106 + t15591 * t1737 / F::cast_from(1536.0_f64) + t5002 * t5014 / F::cast_from(1536.0_f64) - t15594 * t1748 / F::cast_from(2304.0_f64) - t5005 * t5030 / F::cast_from(2304.0_f64);
    t19117
}
