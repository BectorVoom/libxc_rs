//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1434/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1434<F: Float>(t1: F, t1027: F, t1239: F, t935: F, t18023: F, t4477: F, t1162: F, t1179: F, t11937: F, t12567: F, t12568: F, t1508: F, t15781: F, t15860: F, t15911: F, t18016: F, t18043: F, t18069: F, t27382: F, t27815: F, t3103: F, t3234: F, t3235: F, t35724: F, t4435: F, t45442: F, t5312: F, t5375: F, t5399: F, t54066: F, t54111: F, t54754: F, t55761: F, t59462: F, t59503: F, t59637: F, t59762: F, t9122: F, t9123: F, t914: F) -> (F, F) {
    let t59848 = t935 * t1027 * t1239 * t1;
    let t59855 = t18023 * t4477;
    let t59880 = -F::cast_from(0.25786896200974881756e5_f64) * t45442 * t5399 - F::cast_from(0.60470085650610269407e6_f64) * t35724 * t18069 + F::cast_from(0.1699996024669801536e1_f64) * t15911 * t5375 - F::cast_from(0.30228422675018518374e-1_f64) * t1179 * t59503 + F::cast_from(0.25190352229182098644e-1_f64) * t1179 * t59637 + F::cast_from(0.45352564237957702055e6_f64) * t27815 * t54754 * t59848 - F::cast_from(0.45352564237957702055e6_f64) * t27382 * t55761 * t59848 + F::cast_from(0.6058720680803250206e1_f64) * t12567 * t12568 * t59855 + F::cast_from(0.33037286659193699704e3_f64) * t11937 * t18016 - F::cast_from(0.4678438591588217436e2_f64) * t3234 * t3235 * t59762 - F::cast_from(0.20408653907080965924e7_f64) * t9122 * t15781 * t9123 * t5312 - F::cast_from(0.14866778996637164867e4_f64) * t15860 * t18043 + F::cast_from(0.9291736872898228042e2_f64) * t4435 * t54066 * t54111 * t1 - F::cast_from(0.30972456242994093473e2_f64) * t3103 * t54066 * t1508 + F::cast_from(0.30050434779516693818e0_f64) * t1162 * t914 * t59462;
    (t59855, t59880)
}
