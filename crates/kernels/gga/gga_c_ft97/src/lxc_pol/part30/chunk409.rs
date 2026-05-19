//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 409/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk409<F: Float>(t6045: F, t6832: F, t1111: F, t1412: F, t1417: F, t1701: F, t238: F, t3759: F, t3766: F, t3774: F, t6034: F, t6043: F, t6053: F, t6055: F, t6759: F, t6763: F, t6767: F, t6774: F, t6778: F, t6780: F, t6784: F, t6785: F, t6795: F, t6799: F, t6805: F, t6808: F, t6809: F, t6815: F, t6821: F, t6825: F, t6829: F) -> (F, F) {
    let t6833 = t6045 * t6832;
    let t6836 = -F::cast_from(0.23254900946437792e-1_f64) * t3759 * t6759 + F::new(2.0) * t6763 + F::cast_from(0.11854761295685025975e-1_f64) * t1412 * t1111 - F::new(2.0) * t3766 * t6767 + F::new(2.0) * t6778 + F::cast_from(0.25845121844514357744e-4_f64) * t3774 * t6780 - F::cast_from(0.44455354858818847408e-2_f64) * t6784 * t1701 * t6785 - F::cast_from(0.52700762016626893448e-4_f64) * t238 * t6795 + F::cast_from(0.22227677429409423704e-2_f64) * t1417 * t6799 + F::cast_from(0.11854761295685025975e-1_f64) * t238 * t6774 + F::cast_from(0.22270151833971792333e-3_f64) * t6034 * t6805 - F::cast_from(0.38306165027777777778e-1_f64) * t6808 * t6045 * t6809 - F::cast_from(0.45411044255742331791e-3_f64) * t6815 * t6821 + F::cast_from(0.38306165027777777778e-1_f64) * t6043 * t6825 + F::cast_from(0.51074886703703703704e-1_f64) * t1417 * t6829 - t6053 - F::cast_from(0.6384360837962962963e-2_f64) * t6055 * t6833;
    (t6833, t6836)
}
