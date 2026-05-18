//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1156/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1156<F: Float>(t39854: F, t10810: F, t2196: F, t7615: F, t10722: F, t8240: F, t11705: F, t6465: F, t2184: F, t24031: F, t3308: F, t1592: F, t24035: F) -> (F, F, F, F, F, F) {
    let t39855 = F::new(0.69345773920434148506e0) * t39854;
    let t39857 = t2196 * t10810 * t7615;
    let t39858 = F::new(0.27738309568173659402e1) * t39857;
    let t39859 = t8240 * t10722;
    let t39863 = t6465 * t11705;
    let t39866 = t2184 * t3308 * t24031;
    let t39869 = t1592 * t3308 * t24035;
    (t39855, t39858, t39859, t39863, t39866, t39869)
}
