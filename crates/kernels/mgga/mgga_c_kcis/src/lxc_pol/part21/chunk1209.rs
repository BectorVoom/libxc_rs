//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1209/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1209<F: Float>(t27006: F, t28190: F, t26993: F, t7788: F, t93163: F, t96318: F, t96321: F, t96324: F, t96327: F, t96330: F, t96333: F, t97056: F, t97253: F, t97366: F, t96339: F, t96345: F) -> (F, F, F) {
    let t97407 = 0.7722800925925925926e-4 * t28190 * t27006;
    let t97411 = 0.19345601851851851852e-2 * t96318 + 0.51588271604938271605e-2 * t96321 + 0.77382407407407407407e-2 * t96324 - 0.23214722222222222222e-2 * t96327 - 0.23214722222222222222e-2 * t96330 - 0.69505208333333333334e-3 * t7788 * t97366 - 0.23214722222222222222e-2 * t96333 - 0.69505208333333333334e-3 * t7788 * t97056 + 0.23168402777777777778e-3 * t28190 * t26993 - t97407 - 0.34752604166666666667e-3 * t7788 * t97253 + 0.20635308641975308642e-2 * t93163;
    let t97420 = 0.10317654320987654321e-2 * t96339;
    let t97422 = 0.30952962962962962962e-2 * t96345;
    (t97411, t97420, t97422)
}
