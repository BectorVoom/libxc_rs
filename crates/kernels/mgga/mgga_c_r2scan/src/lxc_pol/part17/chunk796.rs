//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 796/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk796<F: Float>(t1764: F, t7824: F, t2816: F, t595: F, t637: F, t1734: F, t2758: F, t5986: F, t2461: F, t759: F, t761: F, t2049: F, t955: F) -> (F, F, F, F, F, F) {
    let t7827 = t7824 * t1764;
    let t7829 = t595 * t2816;
    let t7831 = F::cast_from(0.40020429009866666666e-2_f64) * t7829 * t637;
    let t7832 = t2758 * t1734;
    let t7849 = F::new(80.0) * t5986;
    let t7861 = F::new(0.571528e-1) * t759 * t2461 * t761;
    let t7865 = t759 * t955 * t2049;
    (t7827, t7831, t7832, t7849, t7861, t7865)
}
