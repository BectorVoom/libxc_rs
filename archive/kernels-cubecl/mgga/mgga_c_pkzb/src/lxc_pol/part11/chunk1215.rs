//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1215/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1215<F: Float>(t11019: F, t1137: F, t154: F, t17874: F, t17881: F, t17897: F, t17902: F, t2031: F, t2104: F, t2105: F, t2106: F, t21500: F, t21538: F, t21543: F, t25221: F, t25248: F, t25275: F, t25290: F, t25337: F, t25351: F, t25357: F, t26387: F, t276: F, t287: F, t2899: F, t2900: F, t2922: F, t29813: F, t3542: F, t3645: F, t655: F, t742: F, t761: F, t7664: F, t7701: F, t9161: F, t9187: F, t9258: F, t9287: F, t9292: F) -> F {
    let t29858 = -t276 * t154 * t742 * t29813 / F::cast_from(96.0_f64) - F::cast_from(0.85748036236139473947e-3_f64) * t25248 - t21500 + F::cast_from(0.12862205435420921092e-2_f64) * t2922 * t25221 * t9292 + F::cast_from(0.25724410870841842183e-2_f64) * t25275 - F::cast_from(5.0_f64) / F::cast_from(432.0_f64) * t21538 - t21543 - F::cast_from(5.0_f64) / F::cast_from(486.0_f64) * t17874 - t17881 - t25290 / F::cast_from(96.0_f64) - F::cast_from(0.33875767401931644027e-3_f64) * t17897 - t17902 + F::cast_from(0.12862205435420921092e-2_f64) * t2922 * t25221 * t2900 * t2106 + F::cast_from(0.7717323261252552655e-2_f64) * t2899 * t9258 * t2031 * t9287 - F::cast_from(0.38586616306262763275e-2_f64) * t2922 * t9258 * t7701 * t11019 - F::cast_from(0.12862205435420921092e-2_f64) * t7664 * t25337 * t26387 * t3645 + F::cast_from(0.51448821741683684367e-2_f64) * t25351 - F::cast_from(0.1543464652250510531e-1_f64) * t2104 * t25357 * t761 * t3542 * t655 + F::cast_from(0.7717323261252552655e-2_f64) * t2104 * t9258 * t761 * t9187 - F::cast_from(0.12862205435420921092e-2_f64) * t2104 * t2105 * t1137 * t287 * t9161;
    t29858
}
