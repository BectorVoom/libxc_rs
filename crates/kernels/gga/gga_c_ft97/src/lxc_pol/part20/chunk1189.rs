//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1189/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1189<F: Float>(t1701: F, t2735: F, t27494: F, t2726: F, t2035: F, t6979: F, t2719: F, t14868: F, t6027: F, t108819: F, t1208: F, t1472: F, t14729: F, t14760: F, t19132: F, t231: F, t25049: F, t25050: F, t25112: F, t27506: F, t28552: F, t4104: F, t6045: F, t6795: F, t70435: F, t70497: F, t70598: F) -> (F, F, F, F, F, F) {
    let t112082 = t1701 * t27494 * t2735;
    let t112086 = t1701 * t27494 * t2726;
    let t112090 = t2035 * t6979 * t2726;
    let t112107 = t2035 * t6979 * t2719;
    let t112111 = t1701 * t6027 * t14868;
    let t112119 = 0.88904001456790123461e-1 * t28552 * t108819 - 0.12081826776807659559e1 * t1472 * t112082 + 0.24163653553615319118e1 * t14729 * t112086 + 0.21895580739717983994e1 * t70497 * t112090 + 0.40006800655555555556e0 * t25049 * t6045 * t231 * t70598 + 0.20003400327777777778e0 * t25049 * t6045 * t231 * t1208 * t2719 - 0.60010200983333333334e0 * t25112 * t6045 * t231 * t70435 - 0.10947790369858991997e1 * t19132 * t112107 - 0.45306850413028723348e0 * t4104 * t112111 - 0.10668480174814814815e1 * t25049 * t27506 * t25050 + 0.21895580739717983994e1 * t14760 * t6795;
    (t112082, t112086, t112090, t112107, t112111, t112119)
}
