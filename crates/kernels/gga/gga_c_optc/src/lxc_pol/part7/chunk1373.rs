//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1373/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1373<F: Float>(t27333: F, t4457: F, t8975: F, t3236: F, t8420: F, t1162: F, t1179: F, t11971: F, t12741: F, t12743: F, t26122: F, t26981: F, t27096: F, t27101: F, t27113: F, t27131: F, t27191: F, t27196: F, t27297: F, t27299: F, t27307: F, t27309: F, t27318: F, t27328: F, t3103: F, t3105: F, t3106: F, t3107: F, t3151: F, t3244: F, t3245: F, t8459: F, t8461: F, t894: F, t9128: F, t914: F) -> (F, F) {
    let t27335 = t4457 * t27333 * t8975;
    let t27341 = t8420 * t3236;
    let t27345 = -F::cast_from(0.40304563566691357832e-1_f64) * t1179 * t894 * t3151 * t26122 + F::cast_from(0.1343485452223045261e-1_f64) * t27297 + F::cast_from(0.59710464543246456043e-1_f64) * t27299 + F::cast_from(0.50380704458364197288e-2_f64) * t1179 * t27196 + F::cast_from(0.82101888746963877062e-1_f64) * t1179 * t27191 - F::cast_from(0.2686970904446090522e0_f64) * t1179 * t27131 - F::cast_from(0.67174272611152263053e-2_f64) * t27307 - F::cast_from(0.11195712101858710508e-1_f64) * t27309 - F::cast_from(0.69545291918310062836e0_f64) * t1162 * t914 * t27113 - F::cast_from(0.17386322979577515709e0_f64) * t1162 * t914 * t27096 - F::cast_from(0.30909018630360027928e0_f64) * t27318 + F::cast_from(0.13909058383662012568e1_f64) * t1162 * t914 * t27101 - F::cast_from(0.51620760404990155789e2_f64) * t3103 * t8459 * t3105 * t8461 + F::cast_from(0.20195735602677500686e1_f64) * t27328 + F::cast_from(0.59919903910191457566e4_f64) * t9128 * t3106 * t11971 + F::cast_from(0.11721316454988582616e4_f64) * t27335 + F::cast_from(0.18583473745796456084e3_f64) * t12741 * t26981 * t3107 * t12743 + F::cast_from(0.90880810212048753088e1_f64) * t3244 * t3245 * t27341;
    (t27341, t27345)
}
