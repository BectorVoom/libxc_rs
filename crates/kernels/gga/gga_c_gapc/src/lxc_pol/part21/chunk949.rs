//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 949/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk949<F: Float>(t11733: F, t949: F, t1971: F, t9066: F, t2660: F, t8135: F, t11905: F, t18815: F, t11302: F, t15811: F, t18824: F, t7259: F, t8142: F, t33353: F, t33356: F, t33358: F, t33360: F, t33364: F, t33369: F) -> (F, F, F) {
    let t33371 = t11733 * t949;
    let t33373 = t1971 * t9066;
    let t33374 = t2660 * t33373;
    let t33375 = t33374 * t8135;
    let t33377 = t11905 * t18815;
    let t33380 = t15811 * t11302 * t18824;
    let t33383 = t7259 * t33373 * t8142;
    let t33385 = 0.45289771048911752714e-7 * t33353 + 0.52838066223730378166e-7 * t33356 + 0.11594181388521408695e-4 * t33358 - 0.11594181388521408695e-4 * t33360 + 0.35848176214430067278e-9 * t33364 + 0.47342907336462418838e-4 * t33369 + 0.10821235962619981449e-3 * t33371 - 0.33816362383187442026e-5 * t33375 - 0.27173862629347051628e-6 * t33377 - 0.90579542097823505428e-7 * t33380 - 0.16908181191593721013e-5 * t33383;
    (t33373, t33374, t33385)
}
