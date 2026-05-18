//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 722/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk722<F: Float>(t3116: F, t4757: F, t247: F, t127: F, t1663: F, t371: F, t1025: F, t1063: F, t1068: F, t1675: F, t3106: F, t3112: F, t3127: F, t3174: F, t3188: F, t4818: F, t4821: F, t4825: F, t4831: F, t4834: F, t4837: F) -> (F, F, F, F) {
    let t4838 = t3116 * t4757;
    let t4839 = t247 * t4838;
    let t4845 = t371 * t127 * t1663;
    let t4846 = t1025 * t4845;
    let t4848 = F::new(0.95275595817932748827e-4) * t3112 + F::new(0.14291339372689912324e-3) * t3174 + F::new(0.95275595817932748827e-4) * t4818 + F::new(0.14291339372689912324e-3) * t4821 - F::new(0.14291339372689912324e-3) * t3127 * t4825 + F::new(0.14291339372689912324e-3) * t3188 * t1675 + F::new(0.14291339372689912324e-3) * t1063 * t4831 + F::new(0.14291339372689912324e-3) * t4834 * t1068 + F::new(0.42874018118069736972e-3) * t4837 * t4839 - F::new(0.76220476654346199061e-3) * t3106 * t1675 - F::new(0.14291339372689912324e-3) * t4846;
    (t4839, t4845, t4846, t4848)
}
