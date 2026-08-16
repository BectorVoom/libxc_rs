//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 900/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk900(t28995: f64, t5015: f64, t28368: f64, t5007: f64, t1775: f64, t17248: f64, t17317: f64, t1773: f64, t23326: f64, t23341: f64, t23344: f64, t2460: f64, t28978: f64, t28992: f64, t5013: f64, t7208: f64, t7219: f64, t8798: f64, t8802: f64, t8807: f64, t8811: f64) -> f64 {
    let t28996 = t5015 * t28995;
    let t29001 = t5007 * t28368;
    let t29002 = t1775 * t29001;
    let t29007 = -0.10794473229706390328e0_f64 * t7208 * t8802 - 0.1439263097294185377e0_f64 * t1773 * t28978 - 0.1439263097294185377e0_f64 * t7219 * t8807 - 0.19190174630589138361e0_f64 * t7219 * t8811 + 0.53972366148531951639e-1_f64 * t23326 * t2460 - 0.10794473229706390328e0_f64 * t17317 * t8798 + 0.35981577432354634425e-1_f64 * t23341 - 0.95950873152945691802e-1_f64 * t23344 + 0.10794473229706390328e0_f64 * t5013 * t28992 + 0.10794473229706390328e0_f64 * t5013 * t28996 + 0.28785261945883707541e0_f64 * t17248 * t8798 + 0.10794473229706390328e0_f64 * t1773 * t29002 + 0.28785261945883707541e0_f64 * t7219 * t8802;
    t29007
}
