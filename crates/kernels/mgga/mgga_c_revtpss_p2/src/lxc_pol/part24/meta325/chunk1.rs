//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1131/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1131<F: Float>(t22893: F, t5674: F, t9955: F, t13858: F, t13949: F, t13956: F, t22103: F, t22127: F, t22131: F, t22877: F, t22881: F, t22886: F, t22890: F, t3934: F, t3944: F, t9748: F, t9786: F, t9791: F, t9804: F) -> (F, F) {
    let t22895 = t9955 * t5674 * t22893;
    let t22903 = -t9748 * t22877 / F::new(4.0) - F::cast_from(0.64311027177104605458e-3_f64) * t3934 * t22881 + F::cast_from(0.30492001685571196935e-3_f64) * t22103 + F::new(3.0) / F::new(16.0) * t3944 * t22886 + F::cast_from(0.25724410870841842183e-2_f64) * t3934 * t22890 - F::cast_from(0.12862205435420921092e-1_f64) * t3934 * t22895 + F::cast_from(0.85748036236139473944e-4_f64) * t22127 - F::cast_from(0.42874018118069736972e-3_f64) * t22131 - F::cast_from(0.13553694749236397037e-4_f64) * t13858 - t9786 - t9791 - F::cast_from(0.91464571985215438873e-3_f64) * t13949 + F::cast_from(0.76230004213927992336e-5_f64) * t13956 + t9804;
    (t22895, t22903)
}
