//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1036/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1036(t27720: f64, t7453: f64, t35420: f64, t6051: f64, t35481: f64, t109246: f64, t123181: f64, t13411: f64, t150731: f64, t150764: f64, t150770: f64, t150773: f64, t150776: f64, t150787: f64, t27616: f64, t27665: f64, t27672: f64, t33368: f64, t33385: f64, t33404: f64, t33418: f64, t36835: f64, t3774: f64, t66563: f64) -> f64 {
    let t150789 = t7453 * t27720;
    let t150791 = t35420 * t6051;
    let t150793 = t35481 * t6051;
    let t150797 = 0.25845121844514357744e-4_f64 * t33418 * t150764 - 0.60102574844279699039e-6_f64 * t13411 * t66563 * t150770 - 0.25845121844514357744e-4_f64 * t150773 * t33385 + 0.89080607335887169333e-3_f64 * t150776 * t33368 - 0.20715606998445758511e-4_f64 * t123181 * t36835 * t109246 * t27672 - 0.39601100101559655353e-5_f64 * t27616 * t33404 * t27665 - 0.20869152414369355073e-1_f64 * t150787 + 0.6809984893827160494e-1_f64 * t150789 - 0.25537443351851851852e-1_f64 * t150791 - 0.25537443351851851852e-1_f64 * t150793 - 0.51690243689028715488e-5_f64 * t3774 * t150731;
    t150797
}
