//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 357/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk357(t2258: f64, t379: f64, t5579: f64, t1295: f64, t1300: f64, t1603: f64, t1669: f64, t1701: f64, t399: f64, t5514: f64, t5518: f64, t5523: f64, t5530: f64, t5534: f64, t5538: f64, t5541: f64, t5545: f64, t5547: f64, t5557: f64, t5561: f64, t5569: f64, t5573: f64, t5577: f64, t5580: f64, t5587: f64, t5593: f64, t5598: f64, t5600: f64, t5604: f64, t5610: f64, t5611: f64, t79: f64) -> (f64, f64, f64) {
    let t5612 = t2258 * t379;
    let t5613 = t5579 * t5612;
    let t5616 = -0.23254900946437792e-1_f64 * t1603 * t5514 + 2.0_f64 * t5518 + 0.11854761295685025975e-1_f64 * t1295 * t399 - 2.0_f64 * t1669 * t5523 + 2.0_f64 * t5534 + 0.25845121844514357744e-4_f64 * t5538 * t5541 - 0.44455354858818847408e-2_f64 * t5545 * t1701 * t5547 - 0.52700762016626893448e-4_f64 * t79 * t5557 + 0.22227677429409423704e-2_f64 * t1300 * t5561 + 0.11854761295685025975e-1_f64 * t79 * t5530 + 0.22270151833971792333e-3_f64 * t5569 * t5573 - 0.38306165027777777778e-1_f64 * t5577 * t5579 * t5580 - 0.45411044255742331791e-3_f64 * t5587 * t5593 + 0.38306165027777777778e-1_f64 * t5598 * t5600 + 0.51074886703703703704e-1_f64 * t1300 * t5604 - t5610 - 0.6384360837962962963e-2_f64 * t5611 * t5613;
    (t5612, t5613, t5616)
}
