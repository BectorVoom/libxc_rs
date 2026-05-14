//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 778/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk778<F: Float>(t1664: F, t28357: F, t4742: F, t2382: F, t8573: F, t4704: F, t22801: F, t2381: F, t10696: F, t28341: F, t10699: F, t10690: F, t4790: F, t10639: F, t15989: F, t16389: F, t22564: F, t22575: F, t22583: F, t22698: F, t22705: F, t22707: F, t28362: F, t28379: F, t28387: F, t28394: F, t28404: F) -> (F, F, F, F, F, F) {
    let t28462 = t28357 * t1664;
    let t28464 = 6.0 * t4742 * t28462;
    let t28465 = t2382 * t8573;
    let t28467 = 6.0 * t4704 * t28465;
    let t28468 = t22801 * t2381;
    let t28470 = 0.48245472966453314466e2 * t4742 * t28468;
    let t28471 = t10696 * t28341;
    let t28472 = t28471 * t10699;
    let t28475 = t10690 * t28341;
    let t28476 = t28475 * t4790;
    let t28492 = -0.40256666666666666668e0 * t15989 + 0.247573125e0 * t28362 + 0.258925e1 * t28394 - t10639 - 0.27595e0 * t16389 + 0.5519e-1 * t22698 + 0.20128333333333333333e0 * t22564 - 0.60385000000000000001e0 * t22575 + 0.30192500000000000001e0 * t22583 - 0.33114e0 * t22705 + 0.16557e0 * t22707 + 0.49671e0 * t28404 - 0.60384999999999999999e0 * t28379 + 0.181155e1 * t28387;
    (t28464, t28467, t28470, t28472, t28476, t28492)
}
