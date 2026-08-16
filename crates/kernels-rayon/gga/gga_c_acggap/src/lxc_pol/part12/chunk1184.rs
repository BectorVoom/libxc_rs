//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1184/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1184(t30793: f64, t30798: f64, t30800: f64, t30804: f64, t30809: f64, t30812: f64, t30814: f64, t30818: f64, t30821: f64, t30830: f64, t30840: f64, t30844: f64, t30846: f64, t30848: f64, t30852: f64, t30854: f64, t35001: f64) -> f64 {
    let t37344 = -t35001 / 48.0_f64 - 0.75475421495049964964e-2_f64 * t30793 - 0.42874018118069736972e-3_f64 * t30798 + 0.34299214494455789578e-2_f64 * t30800 + 0.37737710747524982483e-2_f64 * t30804 + 0.94344276868812456204e-2_f64 * t30809 + 0.13719685797782315831e-1_f64 * t30812 - 0.13719685797782315831e-1_f64 * t30814 + 0.37737710747524982482e-2_f64 * t30818 + 0.28582678745379824648e-2_f64 * t30821 - 0.41930789719472202758e-2_f64 * t30830 - 0.25724410870841842184e-2_f64 * t30840 - 0.16006300097412701803e-1_f64 * t30844 + 0.62896184579208304137e-2_f64 * t30846 - 0.94344276868812456206e-2_f64 * t30848 - 0.15724046144802076034e-2_f64 * t30852 + 0.51448821741683684368e-2_f64 * t30854;
    t37344
}
