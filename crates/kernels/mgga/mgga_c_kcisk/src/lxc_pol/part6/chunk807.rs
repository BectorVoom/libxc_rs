//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 807/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk807<F: Float>(t11179: F, t28991: F, t2464: F, t8514: F, t5015: F, t28368: F, t5007: F, t1775: F, t17248: F, t17317: F, t1773: F, t23326: F, t23341: F, t23344: F, t2460: F, t28978: F, t5013: F, t7208: F, t7219: F, t8798: F, t8802: F, t8807: F, t8811: F) -> (F,) {
    let t28992 = t11179 * t28991;
    let t28995 = t8514 * t2464;
    let t28996 = t5015 * t28995;
    let t29001 = t5007 * t28368;
    let t29002 = t1775 * t29001;
    let t29007 = -0.10794473229706390328e0 * t7208 * t8802 - 0.1439263097294185377e0 * t1773 * t28978 - 0.1439263097294185377e0 * t7219 * t8807 - 0.19190174630589138361e0 * t7219 * t8811 + 0.53972366148531951639e-1 * t23326 * t2460 - 0.10794473229706390328e0 * t17317 * t8798 + 0.35981577432354634425e-1 * t23341 - 0.95950873152945691802e-1 * t23344 + 0.10794473229706390328e0 * t5013 * t28992 + 0.10794473229706390328e0 * t5013 * t28996 + 0.28785261945883707541e0 * t17248 * t8798 + 0.10794473229706390328e0 * t1773 * t29002 + 0.28785261945883707541e0 * t7219 * t8802;
    (t29007,)
}
