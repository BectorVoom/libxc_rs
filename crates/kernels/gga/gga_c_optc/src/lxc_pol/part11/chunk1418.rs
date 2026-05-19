//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1418/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1418<F: Float>(t5154: F, t1067: F, t1075: F, t1094: F, t12223: F, t12238: F, t17803: F, t17835: F, t17843: F, t17848: F, t26593: F, t26665: F, t26667: F, t2935: F, t2974: F, t2976: F, t3035: F, t3059: F, t3061: F, t34801: F, t4087: F, t44914: F, t45045: F, t5123: F, t5158: F, t58311: F, t58880: F, t59263: F, t59281: F, t59294: F, t59310: F, t59325: F, t8765: F, t8772: F, t8786: F, t8848: F, t8850: F) -> F {
    let t59348 = t5154 * t5154;
    let t59367 = F::cast_from(0.19298809906722418784e3_f64) * t44914 * t5158 + F::new(4.0) * t4087 * t17835 - F::cast_from(0.24829604254387158296e5_f64) * t26593 * t59263 * t8850 + F::new(1.0) * t1067 * (t59281 + t59294 + t59310 + t59325) * t1075 + F::cast_from(0.19965908856856833625e6_f64) * t26665 * t59263 * t26667 - F::cast_from(0.1403573615389248977e2_f64) * t8765 * t58311 * t1094 - F::cast_from(0.35089340384731224426e1_f64) * t3035 * t58880 * t1094 + F::cast_from(0.51947267698127589897e2_f64) * t3059 * t58880 * t3061 + F::new(24.0) * t12238 * t17848 - F::new(24.0) * t8786 * t59263 * t1075 - F::new(6.0) * t2935 * t59348 * t1075 + F::cast_from(0.96494049533612093922e2_f64) * t2974 * t59348 * t2976 + F::cast_from(0.6233672123775310788e3_f64) * t8772 * t58311 * t3061 - F::new(12.0) * t45045 * t5123 - F::cast_from(0.77195239626889675138e3_f64) * t34801 * t17803 + F::cast_from(0.11579285944033451271e4_f64) * t8848 * t59263 * t2976 + F::cast_from(0.14035736153892489771e2_f64) * t12223 * t17843;
    t59367
}
