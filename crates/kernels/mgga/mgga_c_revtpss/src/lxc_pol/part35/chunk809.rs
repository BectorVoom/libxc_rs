//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 809/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk809<F: Float>(t124: F, t22813: F, t800: F, t1883: F, t22079: F, t5673: F, t1872: F, t6816: F, t22046: F, t3936: F, t6869: F, t543: F, t6836: F, t5674: F, t9955: F, t13858: F, t13949: F, t13956: F, t22103: F, t22127: F, t22131: F, t3934: F, t3944: F, t9748: F, t9786: F, t9791: F, t9804: F) -> (F, F, F, F, F) {
    let t22876 = t124 * t22813;
    let t22877 = t800 * t22876;
    let t22881 = t5673 * t22079 * t1883;
    let t22886 = t800 * t1872 * t6816;
    let t22890 = t3936 * t22046 * t6869;
    let t22893 = t543 * t6836;
    let t22895 = t9955 * t5674 * t22893;
    let t22903 = -t9748 * t22877 / 4.0 - 0.64311027177104605458e-3 * t3934 * t22881 + 0.30492001685571196935e-3 * t22103 + 3.0 / 16.0 * t3944 * t22886 + 0.25724410870841842183e-2 * t3934 * t22890 - 0.12862205435420921092e-1 * t3934 * t22895 + 0.85748036236139473944e-4 * t22127 - 0.42874018118069736972e-3 * t22131 - 0.13553694749236397037e-4 * t13858 - t9786 - t9791 - 0.91464571985215438873e-3 * t13949 + 0.76230004213927992336e-5 * t13956 + t9804;
    (t22877, t22881, t22890, t22895, t22903)
}
