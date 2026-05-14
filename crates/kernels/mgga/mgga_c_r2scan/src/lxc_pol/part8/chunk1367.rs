//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1367/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1367<F: Float>(t21125: F, t21129: F, t21133: F, t21136: F, t21139: F, t21149: F, t21151: F, t21155: F, t21159: F, t21168: F, t26400: F, t26406: F, t28522: F, t28525: F, t21173: F, t21176: F, t21179: F, t21183: F, t21186: F, t21191: F, t21195: F, t21200: F, t21210: F, t21216: F, t26424: F, t26427: F, t26430: F) -> (F, F) {
    let t33439 = 0.65061487801810439052e-1 * t26400 - t26406 - t21125 - t21129 - t21133 - t21136 + 0.2401225740592e-1 * t28522 + 0.4802451481184e-1 * t28525 - t21139 + t21149 - 0.31168546390226634765e3 * t21151 + t21155 - t21159 - 0.3601838610888e-1 * t21168;
    let t33447 = -t21173 + t21176 + 0.21687162600603479684e-1 * t21179 + t21183 + 0.67745118933333333331e-2 * t21186 + t21191 + t21195 - t21200 + 360.0 * t26424 - 0.20575008e1 * t26427 - 0.1714584e0 * t26430 - t21210 - t21216;
    (t33439, t33447)
}
