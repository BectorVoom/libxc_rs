//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 847/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk847(t5459: f64, t5463: f64, t5467: f64, t5474: f64, t5479: f64, t5846: f64, t5847: f64, t5853: f64, t7776: f64, t7781: f64, t7785: f64, t5585: f64, t5601: f64, t5605: f64, t5609: f64, t5612: f64, t5614: f64, t5669: f64, t5855: f64, t5864: f64, t5868: f64, t7795: f64, t7796: f64) -> (f64, f64) {
    let t8961 = -t5459 + t5463 + t5467 - 0.22581706311111111111e-2_f64 * t7776 + t7781 + 0.84681398666666666665e-3_f64 * t7785 + t5474 - t5479 - t5846 - 12.0_f64 * t5847 + t5853;
    let t8964 = -t5585 - 0.571528e-1_f64 * t5855 - t5864 - t5601 - t5605 + t5609 + t5612 - t5614 - 2.0_f64 * t7795 + t5868 + t7796 - t5669;
    (t8961, t8964)
}
