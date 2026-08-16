//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 403/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk403(t1800: f64, t2000: f64, t1972: f64, t477: f64, t443: f64, t485: f64, t1754: f64, t1765: f64, t1684: f64, t1735: f64, t1732: f64, t1738: f64, t1762: f64, t1769: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2001 = t2000 * t1800;
    let t2002 = 22.07984838129906_f64 * t2001;
    let t2003 = t1972 * t477;
    let t2005 = t443 * t443;
    let t2006 = 1.0_f64 / t2005;
    let t2007 = t2006 * t485;
    let t2008 = 0.22687409291590604_f64 * t1754;
    let t2010 = 0.07562469763863536_f64 * t1765;
    let t2012 = 0.04525483399593904_f64 * t1684;
    let t2014 = 0.015084944665313014_f64 * t1735;
    let t2016 = t2008 - 0.22687409291590604_f64 * t1762 + t2010 + 0.22687409291590604_f64 * t1769 + t2012 - 0.04525483399593904_f64 * t1732 + t2014 + 0.04525483399593904_f64 * t1738;
    (t2001, t2002, t2003, t2005, t2006, t2007, t2008, t2010, t2012, t2014, t2016)
}
