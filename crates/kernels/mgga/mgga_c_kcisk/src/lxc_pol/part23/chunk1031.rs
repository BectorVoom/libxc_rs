//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1031/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1031<F: Float>(t14059: F, t14062: F, t14085: F, t14093: F, t14100: F, t173: F, t18953: F, t19033: F, t19087: F, t20670: F, t20676: F, t20679: F, t20687: F, t20688: F, t20691: F, t20694: F, t20697: F, t20700: F, t20703: F, t20706: F, t3819: F, t3891: F, t5816: F, t5823: F, t5827: F) -> (F,) {
    let t20709 = t14059 - t14062 + t20670 - 0.23911438650126355246e-1 * t3819 * t18953 - 0.95645754600505420984e-1 * t14100 * t19087 - t20676 + 0.15538616723388920628e-3 * t3891 * t18953 + 0.62154466893555682512e-3 * t20679 * t19087 + 0.71734315950379065738e-1 * t14093 * t19033 - 0.62154466893555682512e-3 * t14085 * t19033 + t20687 + 0.30247875e-4 * t173 * t20688 + 0.28104e-1 * t5827 * t20691 + 0.4684e-2 * t5827 * t20694 - 0.634e-2 * t5816 * t20697 - 0.21133333333333333334e-2 * t5816 * t20700 - 0.403305e-4 * t5823 * t20703 + 0.403305e-4 * t173 * t20706;
    (t20709,)
}
