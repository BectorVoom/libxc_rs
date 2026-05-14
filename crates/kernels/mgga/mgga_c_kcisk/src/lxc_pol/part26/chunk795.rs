//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 795/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk795<F: Float>(t12476: F, t2872: F, t12485: F, t824: F, t12489: F, t12491: F, t12493: F, t12524: F, t5680: F, t5744: F, t830: F, t815: F, t2901: F, t846: F) -> (F, F, F, F) {
    let t12526 = t2872 * t12476;
    let t12528 = t824 * t12485;
    let t12531 = -0.25319e1 * t12489 + 0.16879333333333333333e1 * t12491 - 0.19692555555555555555e1 * t12493 - 0.93011851851851851854e0 * t5680 + 0.13651666666666666667e0 * t12524 - 0.27303333333333333333e0 * t12526 - 0.3185388888888888889e0 * t12528 - 0.36514074074074074075e0 * t5744;
    let t12532 = t12531 * t830;
    let t12534 = 1.0 * t815 * t12532;
    let t12535 = t2901 * t846;
    (t12526, t12528, t12534, t12535)
}
