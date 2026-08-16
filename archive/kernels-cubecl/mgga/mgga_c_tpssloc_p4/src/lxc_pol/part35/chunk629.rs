//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 629/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk629<F: Float>(t225: F, t5848: F, t68: F, t369: F, t1539: F, t1616: F, t3071: F, t1020: F, t1041: F, t1618: F, t1622: F, t3039: F, t3070: F, t3084: F, t3130: F, t3160: F, t378: F, t4572: F, t4604: F, t4625: F, t4631: F, t4641: F, t4644: F, t5857: F, t5861: F, t5869: F, t5875: F, t5880: F, t5885: F, t5890: F, t5894: F, t5900: F, t973: F) -> (F, F, F, F, F, F) {
    let t5903 = t5848 * t225;
    let t5904 = t5903 * t68;
    let t5905 = t5904 * t369;
    let t5908 = t1616 * t1539;
    let t5909 = t3071 * t5908;
    let t5914 = t1041 * t5857 / F::cast_from(4608.0_f64) + F::cast_from(5.0_f64) / F::cast_from(13824.0_f64) * t1041 * t5861 + t4644 * t1622 / F::cast_from(2304.0_f64) + t1020 * t5869 / F::cast_from(3072.0_f64) + t3130 * t5875 / F::cast_from(1536.0_f64) - t3039 * t5880 / F::cast_from(3072.0_f64) - t3160 + t4625 / F::cast_from(2304.0_f64) - t973 * t5885 / F::cast_from(144.0_f64) + t4604 / F::cast_from(432.0_f64) + t973 * t5890 / F::cast_from(288.0_f64) + t973 * t5894 / F::cast_from(216.0_f64) + t4572 / F::cast_from(3456.0_f64) + t4631 / F::cast_from(2304.0_f64) - t1041 * t5900 / F::cast_from(2304.0_f64) - t3084 + t5905 * t378 / F::cast_from(3072.0_f64) + t3070 * t5909 / F::cast_from(2304.0_f64) + t4641 * t1618 / F::cast_from(1536.0_f64);
    (t5903, t5904, t5905, t5908, t5909, t5914)
}
