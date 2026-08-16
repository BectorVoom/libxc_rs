//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 945/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk945(t22893: f64, t5674: f64, t9955: f64, t13858: f64, t13949: f64, t13956: f64, t22103: f64, t22127: f64, t22131: f64, t22877: f64, t22881: f64, t22886: f64, t22890: f64, t3934: f64, t3944: f64, t9748: f64, t9786: f64, t9791: f64, t9804: f64) -> (f64, f64) {
    let t22895 = t9955 * t5674 * t22893;
    let t22903 = -t9748 * t22877 / 4.0_f64 - 0.64311027177104605458e-3_f64 * t3934 * t22881 + 0.30492001685571196935e-3_f64 * t22103 + 3.0_f64 / 16.0_f64 * t3944 * t22886 + 0.25724410870841842183e-2_f64 * t3934 * t22890 - 0.12862205435420921092e-1_f64 * t3934 * t22895 + 0.85748036236139473944e-4_f64 * t22127 - 0.42874018118069736972e-3_f64 * t22131 - 0.13553694749236397037e-4_f64 * t13858 - t9786 - t9791 - 0.91464571985215438873e-3_f64 * t13949 + 0.76230004213927992336e-5_f64 * t13956 + t9804;
    (t22895, t22903)
}
