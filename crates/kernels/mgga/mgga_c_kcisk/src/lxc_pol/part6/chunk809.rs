//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 809/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk809<F: Float>(t28377: F, t7234: F, t2448: F, t8832: F, t20: F, t28957: F, t649: F, t10795: F, t1773: F, t23338: F, t23769: F, t2460: F, t29011: F, t29017: F, t29025: F, t29029: F, t29032: F, t29036: F, t5013: F, t664: F, t7208: F, t8807: F, t8811: F) -> (F,) {
    let t29039 = t7234 * t28377;
    let t29042 = t2448 * t8832;
    let t29045 = t28957 * t20;
    let t29046 = t649 * t29045;
    let t29049 = 0.53972366148531951639e-1 * t7208 * t8807 + 0.17990788716177317213e-1 * t1773 * t29011 + 0.71963154864709268852e-1 * t7208 * t8811 + 0.55971342672551653552e-1 * t1773 * t29017 + 0.52772980234120130492e0 * t23769 * t2460 - 0.28785261945883707541e0 * t23338 * t2460 + 0.32383419689119170984e0 * t1773 * t29025 - 0.53972366148531951639e-1 * t5013 * t29029 - 0.10794473229706390328e0 * t5013 * t29032 - 0.71963154864709268852e-1 * t5013 * t29036 + 0.71963154864709268853e-1 * t5013 * t29039 + t10795 + 0.15831894070236039148e1 * t29042 * t664 - 0.24627390775922727564e1 * t29046 * t664;
    (t29049,)
}
