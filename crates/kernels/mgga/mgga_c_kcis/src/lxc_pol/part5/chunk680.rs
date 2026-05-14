//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 680/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk680<F: Float>(t1268: F, t5341: F, t1239: F, t1844: F, t1240: F, t1269: F, t1857: F, t3248: F, t3638: F, t3644: F, t3658: F, t4987: F, t4990: F, t4997: F, t5001: F, t5003: F, t5007: F, t5011: F, t5015: F, t5017: F, t5021: F, t5023: F, t5028: F, t5031: F, t5282: F) -> (F, F, F) {
    let t5342 = t5341 * t1268;
    let t5345 = t1844 * t1239;
    let t5357 = 0.890445125e-2 * t3644 * t5282 + 0.66725e-1 * t1240 * t5282 - t3658 - 0.30952962962962962963e-2 * t3248 - 0.11607361111111111111e-2 * t4987 + 0.11607361111111111111e-2 * t4990 + 0.23214722222222222222e-2 * t4997 + 0.11607361111111111111e-2 * t5001 + 0.77382407407407407407e-3 * t5003 - 0.30952962962962962963e-2 * t5007 - 0.66725e-1 * t1240 * t5342 - 0.66725e-1 * t5345 * t1269 - 0.66725e-1 * t3638 * t1857 + 0.11607361111111111111e-2 * t5011 - 0.30952962962962962963e-2 * t5015 - 0.11607361111111111111e-2 * t5017 + 0.46429444444444444443e-2 * t5021 + 0.77382407407407407407e-3 * t5023 - 0.17411041666666666666e-2 * t5028 + 0.11607361111111111111e-2 * t5031;
    (t5342, t5345, t5357)
}
