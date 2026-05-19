//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 847/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk847<F: Float>(t5459: F, t5463: F, t5467: F, t5474: F, t5479: F, t5846: F, t5847: F, t5853: F, t7776: F, t7781: F, t7785: F, t5585: F, t5601: F, t5605: F, t5609: F, t5612: F, t5614: F, t5669: F, t5855: F, t5864: F, t5868: F, t7795: F, t7796: F) -> (F, F) {
    let t8961 = -t5459 + t5463 + t5467 - F::cast_from(0.22581706311111111111e-2_f64) * t7776 + t7781 + F::cast_from(0.84681398666666666665e-3_f64) * t7785 + t5474 - t5479 - t5846 - F::new(12.0) * t5847 + t5853;
    let t8964 = -t5585 - F::new(0.571528e-1) * t5855 - t5864 - t5601 - t5605 + t5609 + t5612 - t5614 - F::new(2.0) * t7795 + t5868 + t7796 - t5669;
    (t8961, t8964)
}
