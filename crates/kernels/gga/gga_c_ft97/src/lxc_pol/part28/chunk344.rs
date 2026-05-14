//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 344/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk344<F: Float>(t2258: F, t379: F, t5579: F, t1295: F, t1300: F, t1603: F, t1669: F, t1701: F, t399: F, t5514: F, t5518: F, t5523: F, t5530: F, t5534: F, t5538: F, t5541: F, t5545: F, t5547: F, t5557: F, t5561: F, t5569: F, t5573: F, t5577: F, t5580: F, t5587: F, t5593: F, t5598: F, t5600: F, t5604: F, t5610: F, t5611: F, t79: F) -> (F, F, F) {
    let t5612 = t2258 * t379;
    let t5613 = t5579 * t5612;
    let t5616 = -0.23254900946437792e-1 * t1603 * t5514 + 2.0 * t5518 + 0.11854761295685025975e-1 * t1295 * t399 - 2.0 * t1669 * t5523 + 2.0 * t5534 + 0.25845121844514357744e-4 * t5538 * t5541 - 0.44455354858818847408e-2 * t5545 * t1701 * t5547 - 0.52700762016626893448e-4 * t79 * t5557 + 0.22227677429409423704e-2 * t1300 * t5561 + 0.11854761295685025975e-1 * t79 * t5530 + 0.22270151833971792333e-3 * t5569 * t5573 - 0.38306165027777777778e-1 * t5577 * t5579 * t5580 - 0.45411044255742331791e-3 * t5587 * t5593 + 0.38306165027777777778e-1 * t5598 * t5600 + 0.51074886703703703704e-1 * t1300 * t5604 - t5610 - 0.6384360837962962963e-2 * t5611 * t5613;
    (t5612, t5613, t5616)
}
