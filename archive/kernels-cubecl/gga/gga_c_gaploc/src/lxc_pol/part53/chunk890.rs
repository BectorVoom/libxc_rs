//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 890/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk890<F: Float>(t13206: F, t7137: F, t1841: F, t3487: F, t734: F, t9636: F, t2558: F, t32743: F, t9647: F, t6118: F, t9755: F, t10809: F, t161: F, t9744: F) -> (F, F, F, F, F) {
    let t43019 = F::cast_from(0.20508069947045931423e-1_f64) * t7137 * t13206;
    let t43023 = F::cast_from(0.85450291446024714263e-3_f64) * t1841 * t9636 * t3487 * t734;
    let t43027 = t9647 * t32743 * t2558;
    let t43028 = F::cast_from(0.64087718584518535698e-3_f64) * t43027;
    let t43032 = F::cast_from(0.59815204012217299984e-2_f64) * t1841 * t9755 * t3487 * t6118;
    let t43040 = F::cast_from(0.10254034973522965711e-1_f64) * t1841 * t10809 * t161 * t9744;
    (t43019, t43023, t43028, t43032, t43040)
}
